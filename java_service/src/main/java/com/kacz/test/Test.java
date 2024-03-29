package com.kacz.test;

import java.util.List;
import java.util.Map;

public class Test {
    private final String name;

    public Test(String newName) {
        this.name = newName;
    }

    public native String hello();

    public native void differentHellos(List<String> names);

    public native void frequencies(String input);
}
