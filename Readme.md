Shitlink - the Shitty Link Shortener


Inspired by poop.is, I've decided to make a shitty link shortener.

The design goal is as follows:

* POST shitlink.com/myshittylink.com
* myshittylink.com gets appended as the last line in a flat text file
* shitlink.com responds to the post with the line number from previous step
* GET shitlink.com/0 redirects you to myshittylink.com

Once I achieve a proof-of-concept, I would like to write the app using tokio, and then tokio_uring.