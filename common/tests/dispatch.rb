

require_relative 'boot_test'

qemu_cmd = ARGV.join(' ')
binary = ARGV.last

test_name = binary.gsub(%r{.*deps/}, '').split('-')[0]

puts "Test name"

case test_name
when 'kernel8.img'
  load 'tests/boot_test_string.rb' # provides 'EXPECTED_PRINT'
  BootTest.new(qemu_cmd, EXPECTED_PRINT).run # doesn't return
end
