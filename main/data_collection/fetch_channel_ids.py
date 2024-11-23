import csv
import logging
from googleapiclient.discovery import build

# Initialize YouTube API client
api_key = ""
youtube = build("youtube", "v3", developerKey=api_key)

# Configure logging
logging.basicConfig(
    filename="fetch_videos.log",
    level=logging.INFO,
    format="%(asctime)s - %(levelname)s - %(message)s"
)

def fetch_recent_videos(channel_id):
    logging.info(f"Fetching videos for channel ID: {channel_id}")
    # Request the 10 most recent videos for a channel
    request = youtube.search().list(
        part="snippet",
        channelId=channel_id,
        maxResults=10,
        order="date",
        type="video"  # Ensures we only get videos, not playlists or channels
    )
    try:
        response = request.execute()
        logging.info(f"Received search response for channel {channel_id}")
    except Exception as e:
        logging.error(f"Failed to fetch videos for channel {channel_id}: {e}")
        return []
    
    # Collect video links
    video_links = [
        f"https://www.youtube.com/watch?v={item['id']['videoId']}"
        for item in response.get("items", [])
    ]
    
    if not video_links:
        logging.warning(f"No videos found for channel ID: {channel_id}")
    return video_links
import re


def save_top_videos_to_csv(input_csv="data/channels.csv", output_csv="data/top_channels_videos.csv"):
    logging.info(f"Starting process to save top videos to {output_csv}")
    total_channels = 0
    total_videos_expected = 0
    total_videos_captured = 0

    with open(input_csv, mode="r") as file:
        reader = csv.DictReader(file)
        channel_video_data = {}

        # Read each channel and its ID from the input CSV
        for row in reader:
            channel_name = row["channel_name"]
            channel_id = row["channel_id"]
            logging.info(f"Processing channel: {channel_name} (ID: {channel_id})")
            video_links = fetch_recent_videos(channel_id)
            num_videos = len(video_links)
            percentage_captured = (num_videos / 10) * 100  # Assuming we expect 10 videos per channel

            logging.info(f"Captured {num_videos} videos for channel '{channel_name}' ({percentage_captured}%)")
            channel_video_data[channel_name] = video_links

            # Update totals
            total_channels += 1
            total_videos_expected += 10
            total_videos_captured += num_videos

    # Write results to output CSV
    with open(output_csv, mode="w", newline="") as file:
        writer = csv.writer(file)
        writer.writerow(["channel", "videos"])
        for channel, videos in channel_video_data.items():
            writer.writerow([channel, videos])

    # Calculate overall percentage captured
    overall_percentage = (total_videos_captured / total_videos_expected) * 100 if total_videos_expected else 0

    # Log summary at the end
    logging.info("===== Summary =====")
    logging.info(f"Total channels processed: {total_channels}")
    logging.info(f"Total videos expected: {total_videos_expected}")
    logging.info(f"Total videos captured: {total_videos_captured}")
    logging.info(f"Overall percentage of videos captured: {overall_percentage:.2f}%")
    logging.info("====================")

    print("Process completed. Check fetch_videos.log for details.")

# Run the script
save_top_videos_to_csv()

