# Iced Video Player

![Basic Example Gif](https://github.com/Night-Hunter-NF/iced_video/blob/master/assets/basic_example.gif)

**Gstreamer install instruction [here](https://gitlab.freedesktop.org/gstreamer/gstreamer-rs#installation)**

### Features:

- can play URLS and files
- automatic auto source selecting

### Known Issues:
- slider goes to 0 after releasing when using seek
- styles bad
- when a video finishes it gstreamer panics
### Road Map

- add wasm support
- build custom widget to display opengl textures if possible (faster then getting and displaying RGB)
- optional overlay with controls(WIP)
- option to popout player


### License

Licensed under either

- [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [MIT](http://opensource.org/licenses/MIT)

at your option.