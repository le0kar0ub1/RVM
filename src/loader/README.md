`src/loader`
============

This directory host the image loader. The only format handled is `ELF`.

The loader will parse the given executable and then create a processus image. This image is dumped as `procimg`.

## Primary Loading

* Copy statics headers into process image

* Copy sections at their given addresses in the process image

* Inject program header as segments in supervisor memory

## Dynamic Loading

* Load shared object at arbitrary address behind the static image

* Resolve relocation in the static part to access the dynamic objects

* Inject program headers as segments in supervisor memory

## Finaly

We are done, the image process is ready to be safely executed.