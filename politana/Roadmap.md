# Politana Roadmap

## Next steps

* Transition to a custom packaging script and support static serving
* Politana Debug Bar
* Start drafting documentation

## To do

### Custom packaging script

* Use head resources in generated HTML
* Base path customization
* Asset packaging?

### Performance

* More efficient ForEach moving

### Error handling

* Make sure all errors are passed through to the offending site
    * To do this, make a custom "unwrap" function

### Organization

* Merge font family/variation and text decoration

### New features

* Strings in CSS properties
* Easier way to embed JS code?
* Page titles, integration with NavigationHost
* Dependency injection?
* Spawn async functions that are automatically canceled when the spawner view disappears
* Easier multi-page apps with better SEO and custom build script
* Vdom inspector
* HTML canvas
* Animations?
* More work on interoperability
* Accessibility
* Binding standard HTML input tag types to State
* SSR

### Correctness

* ⚠️ NavigationController::go_back should be history-based, not just one level up.
* Test WebGPUCanvas effects
* Escape CSS property values?
* Review WebGPU
* Test all errors

### Documentation

* Error messages: standardize vocab (component vs view vs element?)

## Resolved

* Check out concurrent state bugs with wasm "concurrency"
    * The concurrent error will only trigger for nested calls. WASM will not preempt except at ".await" boundaries, and our code never calls these.
