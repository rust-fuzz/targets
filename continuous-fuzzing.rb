#!/usr/bin/env ruby

if ARGV.size >= 1
    time_mul = ARGV[0]
else
    time_mul = 10
end

loop do
    system 'cd fuzzer-honggfuzz; cargo update'

    File.open("targets.txt").each do |line|
        target, weight = line.split
        time = weight.to_i * time_mul.to_i
    
        system "HFUZZ_RUN_ARGS=\"--run_time #{time} $HFUZZ_RUN_ARGS\" ./fuzz-with-honggfuzz.sh #{target}"
    end
end

