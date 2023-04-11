# Iced Video Player

![Basic Example Gif](https://github.com/Night-Hunter-NF/iced_video/blob/master/assets/basic_example.gif)

**Gstreamer install instruction [here](https://gitlab.freedesktop.org/gstreamer/gstreamer-rs#installation)**
has been tested on GStreamer 1.22.1

### Features:

- suports all formats supported by gstreamer playbin
- mutiple players at the same time
- premade video overlay with controls

### Known Issues:
- slider goes to 0 after releasing when using seek
- styles need redoing waiting for a new theme widget
- when a video finishes it gstreamer panics
### Road Map

- add wasm backend
- add ffmpeg backend
- add mpv backend
- build custom widget to display opengl textures if possible (faster then getting and displaying RGB)
- optional overlay with controls(WIP)
- option to popout player needs https://github.com/iced-rs/iced/pull/1439


### License

Licensed under either

- [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [MIT](http://opensource.org/licenses/MIT)

at your option.