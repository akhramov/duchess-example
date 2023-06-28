package greeting;

public class Native {
    static native String baseGreeting(String name);

    static {
        System.loadLibrary("greeter");
    }

    public static void main(String[] args){
        baseGreeting("Einar");
    }
}
