import sys
import json
import yt_dlp

def main():
    if len(sys.argv) < 2:
        print(json.dumps({"error": "No URL provided"}))
        sys.exit(1)
    
    url = sys.argv[1]
    ydl_opts = {
        'format': 'bestaudio/best',
        'quiet': True,
        'no_warnings': True,
        'noplaylist': True,
    }
    try:
        with yt_dlp.YoutubeDL(ydl_opts) as ydl:
            info = ydl.extract_info(url, download=False)
            
            if 'requested_formats' in info:
                stream_url = None
                for f in info['requested_formats']:
                    if f.get('url'):
                        stream_url = f['url']
                        break
            else:
                stream_url = info.get('url')
            
            if not stream_url:
                print(json.dumps({"error": "No stream URL found"}))
                sys.exit(1)
            
            safe_info = ydl.sanitize_info({
                "url": stream_url,
                "title": info.get('title'),
                "duration": info.get('duration'),
                "ext": info.get('ext'),
            })
            print(json.dumps(safe_info))
    except Exception as e:
        print(json.dumps({"error": str(e)}))
        sys.exit(1)

if __name__ == "__main__":
    main()