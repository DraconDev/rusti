package cachex

import (
	"testing"
	"time"
)

func TestNew(t *testing.T) {
	cache := New[string](5 * time.Second)
	if cache == nil {
		t.Fatal("New returned nil")
	}
	if cache.ttl != 5*time.Second {
		t.Errorf("TTL = %v, want 5s", cache.ttl)
	}
	
}

func TestCacheSetAndGet(t *testing.T) {
	cache := New[string](5 * time.Second)

	cache.Set("key1", "value1")
	value, ok := cache.Get("key1")

	if !ok {
		t.Error("Get returned false, want true")
	}
	if value != "value1" {
		t.Errorf("Get = %v, want value1", value)
	}
}

func TestCacheGetNonExistent(t *testing.T) {
	cache := New[string](5 * time.Second)

	value, ok := cache.Get("nonexistent")

	if ok {
		t.Error("Get returned true for nonexistent key, want false")
	}
	if value != "" {
		t.Errorf("Get = %v, want empty string", value)
	}
}

func TestCacheExpiration(t *testing.T) {
	cache := New[string](100 * time.Millisecond)

	cache.Set("key1", "value1")

	// Should exist immediately
	if _, ok := cache.Get("key1"); !ok {
		t.Error("Get returned false immediately after Set")
	}

	// Wait for expiration
	time.Sleep(150 * time.Millisecond)

	// Should be expired
	if _, ok := cache.Get("key1"); ok {
		t.Error("Get returned true after expiration, want false")
	}
}

func TestCacheSetWithTTL(t *testing.T) {
	cache := New[string](5 * time.Second)

	cache.SetWithTTL("key1", "value1", 100*time.Millisecond)

	// Should exist immediately
	if _, ok := cache.Get("key1"); !ok {
		t.Error("Get returned false immediately after SetWithTTL")
	}

	// Wait for custom TTL expiration
	time.Sleep(150 * time.Millisecond)

	// Should be expired
	if _, ok := cache.Get("key1"); ok {
		t.Error("Get returned true after custom TTL expiration, want false")
	}
}

func TestCacheDelete(t *testing.T) {
	cache := New[string](5 * time.Second)

	cache.Set("key1", "value1")
	cache.Delete("key1")

	if _, ok := cache.Get("key1"); ok {
		t.Error("Get returned true after Delete, want false")
	}
}

func TestCacheClear(t *testing.T) {
	cache := New[string](5 * time.Second)

	cache.Set("key1", "value1")
	cache.Set("key2", "value2")
	cache.Clear()

	if _, ok := cache.Get("key1"); ok {
		t.Error("Get(key1) returned true after Clear, want false")
	}
	if _, ok := cache.Get("key2"); ok {
		t.Error("Get(key2) returned true after Clear, want false")
	}
}

func TestCacheSize(t *testing.T) {
	cache := New[string](5 * time.Second)

	if size := cache.Size(); size != 0 {
		t.Errorf("Size = %d, want 0", size)
	}

	cache.Set("key1", "value1")
	cache.Set("key2", "value2")

	if size := cache.Size(); size != 2 {
		t.Errorf("Size = %d, want 2", size)
	}
}

func TestCacheCleanup(t *testing.T) {
	cache := New[string](100 * time.Millisecond)

	cache.Set("key1", "value1")
	cache.Set("key2", "value2")

	// Wait for expiration
	time.Sleep(150 * time.Millisecond)

	// Cleanup should remove expired entries
	removed := cache.Cleanup()

	if removed != 2 {
		t.Errorf("Cleanup removed %d entries, want 2", removed)
	}

	if size := cache.Size(); size != 0 {
		t.Errorf("Size after cleanup = %d, want 0", size)
	}
}

func TestCacheCleanupPartial(t *testing.T) {
	cache := New[string](5 * time.Second)

	cache.SetWithTTL("key1", "value1", 100*time.Millisecond)
	cache.Set("key2", "value2") // Uses default 5s TTL

	// Wait for key1 to expire
	time.Sleep(150 * time.Millisecond)

	removed := cache.Cleanup()

	if removed != 1 {
		t.Errorf("Cleanup removed %d entries, want 1", removed)
	}

	if size := cache.Size(); size != 1 {
		t.Errorf("Size after cleanup = %d, want 1", size)
	}

	// key2 should still exist
	if _, ok := cache.Get("key2"); !ok {
		t.Error("Get(key2) returned false after cleanup, want true")
	}
}

func TestCacheStartCleanupRoutine(t *testing.T) {
	cache := New[string](50 * time.Millisecond)

	cache.Set("key1", "value1")
	cache.Set("key2", "value2")

	// Start cleanup routine with 100ms interval
	stop := cache.StartCleanupRoutine(100 * time.Millisecond)
	defer close(stop)

	// Wait for entries to expire and cleanup to run
	time.Sleep(200 * time.Millisecond)

	if size := cache.Size(); size != 0 {
		t.Errorf("Size after cleanup routine = %d, want 0", size)
	}
}

func TestCacheGenericTypes(t *testing.T) {
	// Test with int
	intCache := New[int](5 * time.Second)
	intCache.Set("num", 42)
	if val, ok := intCache.Get("num"); !ok || val != 42 {
		t.Errorf("Int cache: got %v, %v; want 42, true", val, ok)
	}

	// Test with struct
	type Person struct {
		Name string
		Age  int
	}
	personCache := New[Person](5 * time.Second)
	personCache.Set("person1", Person{Name: "Alice", Age: 30})
	if val, ok := personCache.Get("person1"); !ok || val.Name != "Alice" {
		t.Errorf("Struct cache: got %v, %v; want Alice, true", val, ok)
	}

	// Test with pointer
	ptrCache := New[*Person](5 * time.Second)
	person := &Person{Name: "Bob", Age: 25}
	ptrCache.Set("person2", person)
	if val, ok := ptrCache.Get("person2"); !ok || val.Name != "Bob" {
		t.Errorf("Pointer cache: got %v, %v; want Bob, true", val, ok)
	}
}

func TestCacheConcurrency(t *testing.T) {
	cache := New[int](5 * time.Second)
	done := make(chan bool)

	// Concurrent writes
	for i := 0; i < 10; i++ {
		go func(n int) {
			cache.Set("key", n)
			done <- true
		}(i)
	}

	// Concurrent reads
	for i := 0; i < 10; i++ {
		go func() {
			cache.Get("key")
			done <- true
		}()
	}

	// Wait for all goroutines
	for i := 0; i < 20; i++ {
		<-done
	}

	// Should not panic and should have a value
	if _, ok := cache.Get("key"); !ok {
		t.Error("Concurrent operations failed")
	}
}
