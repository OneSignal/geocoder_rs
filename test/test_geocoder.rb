#!/usr/bin/env ruby

require 'minitest/autorun'

#
# Extremely basic unit test for geocoder
#
class TestGeocoder < MiniTest::Test
  def test_geocoder
    load_library
    assert Geocoder.find_country(-122.3164206, 37.5506619), "US"
  end

  def load_library
    if ARGV[0]
      require 'fiddle'
      library = Fiddle.dlopen(ARGV[0])
      func = Fiddle::Function.new(library['init_geocoder'],
                                  [], Fiddle::TYPE_VOIDP)
      func.call
    else
      puts "Requiring geocoder on Ruby #{RbConfig::CONFIG['ruby_version']}"
      require 'geocoder_rs'
    end
  end
end
