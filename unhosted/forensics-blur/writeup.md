
# Text Bluring

* **Event:** UTCTF
* **Problem Type:** Forensics
* **Point Value / Difficulty:** Hard


This is a much better writeup than I could give here for this:
https://bishopfox.com/blog/unredacter-tool-never-pixelation


Useful resources:
- https://bishopfox.com/blog/unredacter-tool-never-pixelation
- https://dheera.net/posts/20140725-why-you-should-never-use-pixelation/
- https://positive.security/blog/video-depixelation
- https://github.com/beurtschipper/Depix


## General Hints:

Font settings:
- (First version) Liberation Sans, 16pt in Chromium on Fedora
- (Second and third versions) Liberation Sans, 15pt (20px) in Chromium on Fedora

The block size is 5px.

The flag is in the second of the two redacted sections of text that aren't
obvious from context.


## Example Solution

Use [unredacter](https://github.com/bishopfox/unredacter), and get it running.

Find the font and font size, get it to match the image; in this case,
Liberation Sans, 15pt; Electron's font rendering should match.

Fix unredacter's color averaging to be more accurate for sRGB values (this will
make the brightness a bit of a closer match); basically squaring the components
before adding them, and the taking the square root of the average:
$$\bar{r} = \sqrt{\frac{\sum_i{r_i^2}}{n}}$$

Get unredacter to use a hardcoded prefix for the flag format (`utflag{`),
and try offsets until you find one that matches; then hardcode that offset (because
unredacter is often bad at picking them itself) and try different thresholds
until you get something.

