import plistlib

d=plistlib.load(open('/Users/malted/Library/Safari/Bookmarks.plist','rb'))

rl=[c for c in d['Children'] if c.get('Title')=='com.apple.ReadingList']

for i in rl['Children']:
    print(i['ReadingList']['DateAdded'], i['URLString'])
