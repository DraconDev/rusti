package configx

import (
	"os"
	"testing"
)

func TestLoad(t *testing.T) {
	// Setup
	os.Setenv("TEST_PORT", "9000")
	os.Setenv("TEST_REQUIRED", "value")
	defer os.Unsetenv("TEST_PORT")
	defer os.Unsetenv("TEST_REQUIRED")

	fields := []ConfigField{
		{
			Key:          "TEST_PORT",
			DefaultValue: "8080",
			Required:     false,
			Description:  "Test port",
		},
		{
			Key:          "TEST_REQUIRED",
			DefaultValue: "",
			Required:     true,
			Description:  "Required field",
		},
		{
			Key:          "TEST_DEFAULT",
			DefaultValue: "default_value",
			Required:     false,
			Description:  "Field with default",
		},
	}

	config, err := Load(fields, DefaultOptions())
	if err != nil {
		t.Fatalf("Load failed: %v", err)
	}

	// Test environment variable override
	if got := config.Get("TEST_PORT"); got != "9000" {
		t.Errorf("Get(TEST_PORT) = %v, want 9000", got)
	}

	// Test required field
	if got := config.Get("TEST_REQUIRED"); got != "value" {
		t.Errorf("Get(TEST_REQUIRED) = %v, want value", got)
	}

	// Test default value
	if got := config.Get("TEST_DEFAULT"); got != "default_value" {
		t.Errorf("Get(TEST_DEFAULT) = %v, want default_value", got)
	}
}

func TestLoadRequiredFieldMissing(t *testing.T) {
	fields := []ConfigField{
		{
			Key:          "MISSING_REQUIRED",
			DefaultValue: "",
			Required:     true,
			Description:  "Required field",
		},
	}

	_, err := Load(fields, DefaultOptions())
	if err == nil {
		t.Error("Load should fail when required field is missing")
	}
}

func TestConfigGet(t *testing.T) {
	fields := []ConfigField{
		{Key: "KEY1", DefaultValue: "value1", Required: false},
	}

	config, _ := Load(fields, DefaultOptions())

	if got := config.Get("KEY1"); got != "value1" {
		t.Errorf("Get(KEY1) = %v, want value1", got)
	}

	if got := config.Get("NONEXISTENT"); got != "" {
		t.Errorf("Get(NONEXISTENT) = %v, want empty string", got)
	}
}

func TestConfigGetOrDefault(t *testing.T) {
	fields := []ConfigField{
		{Key: "KEY1", DefaultValue: "value1", Required: false},
	}

	config, _ := Load(fields, DefaultOptions())

	if got := config.GetOrDefault("KEY1", "fallback"); got != "value1" {
		t.Errorf("GetOrDefault(KEY1) = %v, want value1", got)
	}

	if got := config.GetOrDefault("NONEXISTENT", "fallback"); got != "fallback" {
		t.Errorf("GetOrDefault(NONEXISTENT) = %v, want fallback", got)
	}
}

func TestConfigSet(t *testing.T) {
	fields := []ConfigField{
		{Key: "KEY1", DefaultValue: "value1", Required: false},
	}

	config, _ := Load(fields, DefaultOptions())
	config.Set("KEY1", "new_value")

	if got := config.Get("KEY1"); got != "new_value" {
		t.Errorf("After Set, Get(KEY1) = %v, want new_value", got)
	}

	config.Set("NEW_KEY", "new_value")
	if got := config.Get("NEW_KEY"); got != "new_value" {
		t.Errorf("After Set, Get(NEW_KEY) = %v, want new_value", got)
	}
}

func TestConfigHas(t *testing.T) {
	fields := []ConfigField{
		{Key: "KEY1", DefaultValue: "value1", Required: false},
	}

	config, _ := Load(fields, DefaultOptions())

	if !config.Has("KEY1") {
		t.Error("Has(KEY1) = false, want true")
	}

	if config.Has("NONEXISTENT") {
		t.Error("Has(NONEXISTENT) = true, want false")
	}
}

func TestConfigAll(t *testing.T) {
	fields := []ConfigField{
		{Key: "KEY1", DefaultValue: "value1", Required: false},
		{Key: "KEY2", DefaultValue: "value2", Required: false},
	}

	config, _ := Load(fields, DefaultOptions())
	all := config.All()

	if len(all) != 2 {
		t.Errorf("All() returned %d fields, want 2", len(all))
	}

	if all["KEY1"] != "value1" {
		t.Errorf("All()[KEY1] = %v, want value1", all["KEY1"])
	}

	if all["KEY2"] != "value2" {
		t.Errorf("All()[KEY2] = %v, want value2", all["KEY2"])
	}
}

func TestLoadOptionsFailOnMissing(t *testing.T) {
	opts := &LoadOptions{
		EnvFile:       "nonexistent.env",
		FailOnMissing: true,
	}

	fields := []ConfigField{
		{Key: "KEY1", DefaultValue: "value1", Required: false},
	}

	_, err := Load(fields, opts)
	if err == nil {
		t.Error("Load should fail when FailOnMissing is true and env file doesn't exist")
	}
}
