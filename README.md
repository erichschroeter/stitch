Stitch is a command-line program that stitches images together.

The original need for this program was to work around a lacking feature to set separate desktop backgrounds for multiple monitors.
This allows a single image to be created and set as the desktop background where each monitor will display separate images.

> **Note:** A missing feature is scaling of images to a desired width and height.
All this means, for the time being, is that you need to scale your images before passing them to stitch if you want them to match your monitor dimensions exactly.

# Usage

Assuming you have a multi-monitor setup like below:

- 1600x1200
- 1920x1080

![Multi monitor setup](docs/monitors.png?raw=true "Multi monitor setup")

The following command would create a new image, _wallpaper.png_, to span both monitors where each monitor will only display the respective image.

> **Note:** the `-y 120` is needed because the origin is in the top-left.

    stitch -x 0 -y 0 one.png -x 1600 -y 120 two.png -o wallpaper.png
