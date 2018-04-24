#!/usr/bin/env ruby

if ARGV.size >= 1
    time_mul = ARGV[0]
else
    time_mul = 10
end

loop do
    File.open("targets.txt").each do |line|
        target, weight = line.split
        time = weight.to_i * time_mul.to_i
    
        system "LIBFUZZER_ARGS=\"-max_total_time=#{time} $LIBFUZZER_ARGS\" ./fuzz-with-libfuzzer.sh #{target}"
    end

    system 'cd fuzzer-libfuzzer; cargo update'
end

