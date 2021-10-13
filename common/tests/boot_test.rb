

require_relative 'test'

require 'timeout'

class BootTest < Test
  MAX_WAIT_SECS = 5

  def initialize(qemu_cmd, expected_print)
    super()

    @qemu_cmd = qemu_cmd
    puts qemu_cmd
    @expected_print = expected_print

    @test_name = 'Boot test'
    @test_description = "Checking for the string: '#{@expected_print}'"
    @test_output = []
    @test_error = nil
  end
  private

  def post_process_and_add_output(qemu_output)
    @test_output += qemu_output.join.split("\n")
  end

  def expected_string_observed?(qemu_output)
    qemu_output.join.include?(@expected_string)
  end

  def setup
    @qemu_serial = IO.popen(@qemu_cmd, err: '/dev/null')
    @qemu_pid = @qemu_serial.pid
  end

  def run_concrete_test
    qemu_output = []
    Timeout.timeout(MAX_WAIT_SECS) do
      while IO.select([@qemu_serial])
        qemu_output << @qemu_serial.read_nonblock(1024)
        if expected_string_observed?(qemu_output)
          @test_error = false
          break
        end
      end
    end
  rescue EOFError
    @test_error = 'QEMU quit unexpectedly'
  rescue Timeout::Error
    @test_error = 'Timed out waiting for magic string'
  rescue StandardError => e
    @test_error = e.message
  ensure
    post_process_and_add_output(qemu_output)
  end
end
