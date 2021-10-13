# coding: utf-8


class Test
  INDENT = '             '

  def initialize
    # template instance variables.
    # @test_name
    # @test_description
    # @test_output
    # @test_error

  end

  private

  def print_border(content)
    puts "#{INDENT}-------------------------------------------------------------------"
    puts content
    puts "#{INDENT}-------------------------------------------------------------------"
  end

  def print_header
    print_border("#{INDENT}🦀 #{@test_description}")
    puts
  end

  def print_footer_error(error)
    puts
    print_border("#{INDENT}❌ Failure: #{@test_name}: #{error}")
    puts
    puts
  end

  def print_footer_success
    puts
    print_border("#{INDENT}✅ Success: #{@test_name}")
    puts
    puts
  end

  def print_output
    @test_output.each { |x| print "#{INDENT}#{x}\n" }
  end

  def setup; end
  def cleanup; end

  def run_concrete_test
    raise('Not implemented')
  end

  public
  def run
    setup
    run_concrete_test
    cleanup

    print_header
    print_output

    exit_code = if @test_error
                  print_footer_error(@test_error)
                  false
                else
                  print_footer_success
                  true
                end
    exit(exit_code)
  end
end

