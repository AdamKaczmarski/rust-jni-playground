package com.kacz.test;

import org.apache.commons.lang3.RandomStringUtils;
import org.apache.commons.lang3.time.StopWatch;

import java.time.Instant;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.TimeUnit;

public class Main {
    static {
        System.loadLibrary("rs_handler");
    }

    public static String generateString(int length) {
        return RandomStringUtils.random(length, true, false);
    }

    public static void frequencies(String input) {
        Map<Character, Long> result = new HashMap<>();
        for (int i = 0; i < input.length(); i++) {
            char key = input.charAt(i);
            if (result.containsKey(key)) {
                result.put(key, result.get(key) + 1L);
            } else {
                result.put(key, 1L);
            }
        }
//        return result;
    }

    public static native void rsFrequencies(String input);


    public static void main(String[] args) {
        StopWatch stopWatch = StopWatch.create();
        int elements = 100_000_000;
        String input = generateString(elements);

        stopWatch.start();
        for (int i = 0; i < 10; i++) {
            frequencies(input);
        }
        stopWatch.stop();
        System.out.printf("[JAVA] Counted frequencies for %d elements in %d ms%n", elements, stopWatch.getTime(TimeUnit.MILLISECONDS));

        stopWatch = StopWatch.create();
        stopWatch.start();
        for (int i = 0; i < 10; i++) {
            rsFrequencies(input);
        }
        stopWatch.stop();
        System.out.printf("[RUST] Counted frequencies for %d elements in %d ms%n", elements, stopWatch.getTime(TimeUnit.MILLISECONDS));
    }
}