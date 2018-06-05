# -*- coding: utf-8 -*-
require 'English'

Gem::Specification.new do |s|
  s.name        = 'geocoder'
  s.version     = '1.0.0'
  s.summary     = 'Rust Reverse Geocoder'
  s.description = 'Takes in a coordinate, returns a country code'

  s.authors     = ['Josh Leverette']
  s.email       = 'jleverette@onesignal.com'
  s.homepage    = 'https://github.com/'
  s.license     = 'Proprietary'

  s.extensions    = %w(Rakefile)
  s.files         = `git ls-files`.split($OUTPUT_RECORD_SEPARATOR)
  s.require_paths = %w(lib)
  s.test_files    = %w(test/test_rusty_blank.rb)

  s.add_runtime_dependency 'thermite', '~> 0'
  s.add_development_dependency 'minitest', '~> 5.8'
end