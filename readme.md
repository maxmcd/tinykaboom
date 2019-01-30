```

convert -coalesce  -duplicate 1,-2-1 -delay 0 out-*.ppm spike.mp4

ffmpeg -i spike.mp4  -vcodec libx264 -profile:v main -level 3.1 -preset medium -crf 23 -x264-params ref=4 -acodec copy -movflags +faststart -pix_fmt yuv420p out.mp4

```
