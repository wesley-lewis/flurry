# Concurrent HashMap 

Normally in Rust, hashmaps are used along with mutexes to share the hashmap with different threads. This is good enough for most applications but what if we want more performance. what if there are multiple cores on the CPU that need access to a shared state. 
By using mutexes only 1 thread can access the hashmap at one time. Java's hashmap gives the user the possibility of accessing the hashmap in a safe way among different CPUs/ threads. 

We will be using a bit of unsafe rust.

### How will the system be tested?
The system will be tested in the same way as the test cases written for Java concurrent HashMap. We will be just be using the same implementation of java test.
