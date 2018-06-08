This is a Rust library for offline reverse geocoding of coordinates into country codes with built-in support to act as a Ruby gem.

## building and testing

to install locally:

    gem build geocoder_rs  # builds the `.gem` file
    gem install geocoder_rs-*.gem # installs the built gem file locally

to create a release tarball for upload to GitHub:

    rake thermite:tarball

to run the tests:

    rake test # runs the Rust tests and Ruby tests

## releasing

make sure that the Cargo.toml version and the Gemfile version are the same, commit and push the changes, build a release tarball, go to GitHub and upload the release tarball to a new release tagged like `v1.0.0` (the leading `v` is important).
