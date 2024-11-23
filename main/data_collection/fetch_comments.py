import json
import csv
import logging
import re
from googleapiclient.discovery import build

# Initialize YouTube API client
api_key = ""
youtube = build("youtube", "v3", developerKey=api_key)

# Configure logging
logging.basicConfig(
    filename="fetch_comments.log",
    level=logging.INFO,
    format="%(asctime)s - %(levelname)s - %(message)s"
)

def extract_video_id(url):
    """
    Extracts the video ID from a YouTube URL.
    """
    pattern = r"(?:https?:\/\/)?(?:www\.)?youtube\.com\/watch\?v=([a-zA-Z0-9_-]{11})"
    match = re.search(pattern, url)
    return match.group(1) if match else None

def fetch_and_parse_comments(video_id, max_comments=1000):
    """
    Fetches comment threads from a YouTube video and parses them into a structured format.
    Replies are added to the data as separate entries, formatted like regular comments.
    """
    cleaned_data = []
    next_page_token = None
    total_comments_fetched = 0
    total_errors = 0
    thread_number = 0

    while total_comments_fetched < max_comments:
        try:
            # Fetch comments from the YouTube API
            request = youtube.commentThreads().list(
                part="snippet,replies",
                videoId=video_id,
                maxResults=100,  # Maximum allowed by the API
                pageToken=next_page_token
            )
            response = request.execute()

            # Parse the comments directly from the response
            for item in response.get("items", []):
                thread_number += 1
                # Parse top-level comment
                parsed_comment = parse_reply(item, thread_number)
                if parsed_comment:
                    cleaned_data.append(parsed_comment)

                # Parse replies if available
                if "replies" in item:
                    for reply in item["replies"]["comments"]:
                        parsed_reply = parse_reply(reply, thread_number, is_reply=True)
                        if parsed_reply:
                            cleaned_data.append(parsed_reply)

            # Update total fetched
            total_comments_fetched += len(response.get("items", []))

            # Get the next page token
            next_page_token = response.get("nextPageToken")

            # Break if there's no next page
            if not next_page_token:
                break

        except Exception as e:
            total_errors += 1
            logging.error(f"Error fetching comments for video {video_id}: {e}")
            break

    percentage_captured = (len(cleaned_data) / max_comments) * 100
    logging.info(
        f"Fetched {len(cleaned_data)} comments for video {video_id} "
        f"({percentage_captured:.2f}% captured, {total_errors} errors)"
    )
    return cleaned_data


def parse_reply(json, thread_number, is_reply=False):
    """
    Parses a single comment or reply JSON into a structured dictionary.
    """
    try:
        snippet = json["snippet"] if is_reply else json["snippet"]["topLevelComment"]["snippet"]
        channel_id = snippet["channelId"]
        return {
            "channel_id": channel_id,
            "thread_id": f"{channel_id}_{thread_number}",
            "video_id": snippet["videoId"],
            "author_id": snippet["authorChannelId"]["value"],
            "author_name": snippet["authorDisplayName"],
            "like_count": snippet["likeCount"],
            "reply_count": 0 if is_reply else json["snippet"].get("totalReplyCount", 0),
            "is_reply": is_reply,
            "text_display": snippet["textDisplay"],
            "author_image_url": snippet["authorProfileImageUrl"],
        }
    except KeyError as e:
        logging.error(f"KeyError while parsing thread {thread_number}: {e}")
        return None


def process_csv(input_csv, output_csv, max_comments=1000):
    """
    Processes each video in the 'videos' column of the input CSV,
    fetches up to `max_comments` for each video, and writes the results to an output CSV.
    """
    with open(input_csv, mode="r") as infile, open(output_csv, mode="w", newline="", encoding="utf-8") as outfile:
        reader = csv.DictReader(infile)
        writer = csv.DictWriter(outfile, fieldnames=[
            "channel_name", "video_id", "channel_id", "thread_id", "author_id",
            "author_name", "like_count", "reply_count", "is_reply", "text_display", "author_image_url"
        ])
        writer.writeheader()

        for row in reader:
            channel_name = row["channel"]
            video_urls = row["videos"].split(",")  # Assuming videos are comma-separated

            for video_url in video_urls:
                video_id = extract_video_id(video_url.strip())
                if not video_id:
                    logging.error(f"Invalid video URL: {video_url}")
                    continue

                print(f"Processing channel: {channel_name}, video: {video_id}")
                logging.info(f"Fetching comments for channel: {channel_name}, video ID: {video_id}")

                # Fetch and parse comments
                cleaned_data = fetch_and_parse_comments(video_id, max_comments=max_comments)

                # Add channel name to each row and write to the output CSV
                for comment in cleaned_data:
                    comment["channel_name"] = channel_name
                    comment["video_id"] = video_id
                    writer.writerow(comment)

                print(f"Finished processing video: {video_id}")

    print(f"Comments saved to {output_csv}")


# Example usage
if __name__ == "__main__":
    input_csv = "data/top_channels_videos.csv"  # Input file with columns: channel_name, videos
    output_csv = "data/comments.csv"       # Output file
    max_comments = 1000               # Max comments per video

    process_csv(input_csv, output_csv, max_comments=max_comments)
