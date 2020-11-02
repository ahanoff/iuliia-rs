# iuliia-codegen

This crate used to generate boilerplate for each transliteration schema based on json mapping provided by nagleon

## How to use

### Install crate locally

`cargo install iuliia-codegen`

### Generate transliteration schema

Pass path to schema json file as input

`iuliia-codegen -f gost_7034.json`

Generated schema will be located in `generated` folder with name of schema and .rs extension