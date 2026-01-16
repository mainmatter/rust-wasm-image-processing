export default function myInitializer() {
  return {
    onStart: () => {
      console.log("Loading...");
      console.time("trunk-initializer");
    },
    onProgress: ({ current, total }) => {
      if (!total) {
        console.log("Loading...", current, "bytes");
      } else {
        console.log("Loading...", Math.round((current / total) * 100), "%");
      }
    },
    onComplete: () => {
      console.log("Loading... done!");
      console.timeEnd("trunk-initializer");
    },
    onSuccess: async (wasm) => {
      console.log("Loading... successful!");
      console.log("WebAssembly: ", wasm);

      // Initialize the rayon thread pool for parallel processing
      const numThreads = navigator.hardwareConcurrency || 4;
      console.log(`Initializing thread pool with ${numThreads} threads...`);
      await wasm.initThreadPool(numThreads);
      console.log("Thread pool initialized!");
    },
    onFailure: (error) => {
      console.warn("Loading... failed!", error);
    },
  };
}
