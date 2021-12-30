#!/usr/bin/env python
import yt_dlp
from sys import argv

ydl = yt_dlp.YoutubeDL({"quiet": True, "extract_flat": True})
res = ydl.extract_info(argv[1], download=False)

if 'entries' in res:
    for i in res['entries']:
        print(i['url'])
else:
    print(res['webpage_url'])
