import csv
from googleapiclient.discovery import build

# Initialize YouTube API client
api_key = ""
youtube = build("youtube", "v3", developerKey=api_key)

def get_channel_id(channel_name):
    # Search for the channel by name
    request = youtube.search().list(
        part="snippet",
        q=channel_name,
        type="channel",
        maxResults=1
    )
    response = request.execute()
    
    # Extract the channel ID if a match is found
    items = response.get("items")
    if items:
        return items[0]["snippet"]["channelId"]
    else:
        print(f"Channel not found for: {channel_name}")
        return None

def save_channels_to_csv(channels, filename="data/channels.csv"):
    with open(filename, mode="w", newline="") as file:
        writer = csv.writer(file)
        # Write headers
        writer.writerow(["channel_name", "channel_id"])
        
        # Write each channel and its ID
        for channel_name in channels:
            channel_id = get_channel_id(channel_name)
            if channel_id:
                writer.writerow([channel_name, channel_id])

# Example list of channel names
channels = [
    "Pod Save America",
    "The Ben Shapiro Show",
    "The Joe Rogan Experience",
    "The Dan Bongino Show",
    "The Daily Wire",
    "The Rubin Report",
    "The Young Turks",
    "The Charlie Kirk Show",
    "The Majority Report with Sam Seder",
    "The Sean Hannity Show",
    "The Megyn Kelly Show",
    "The Ezra Klein Show",
    "The New Abnormal",
    "The Matt Walsh Show",
    "The David Pakman Show",
    "The Thom Hartmann Program",
    "The Jimmy Dore Show",
    "The Candace Owens Show",
    "The Rising (The Hill)",
    "Breaking Points with Krystal and Saagar",
    "Left, Right & Center",
    "The Realignment",
    "Chapo Trap House",
    "Reason Roundtable",
    "The Political Gabfest",
    "The Weeds (Vox)",
    "Intercepted",
    "Majority 54 with Jason Kander",
    "The Realignment",
    "The Political Orphanage",
    "H3H3 Podcast",
    "Matt Shane Secret Podcast",
    "Flagrant Podcast",
    "Full Send Podcast",
    "Impaulsive",
    "Hasanabi",
    "This Past Weekend Theo Von",
    "Lex Fridman",
    "Andrew Huberman",
    "Jordan B. Peterson" #This guy sucks!
]

# Fetch and save channel IDs to CSV
save_channels_to_csv(channels)
print("CSV file saved as channels.csv")
