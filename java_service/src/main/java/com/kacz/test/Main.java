package com.kacz.test;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.Map;

public class Main {
    static {
        System.loadLibrary("rs_handler");
    }

    // The rest is just regular ol' Java!
    public static void main(String[] args) throws InterruptedException {
        Test t = new Test("world");
        System.out.println(t.hello());
        ArrayList<String>names = new ArrayList<>(3) ;
        names.add("Adam");
        names.add("Kacz");
        names.add("from Rust");
        t.differentHellos(names);



    }
}