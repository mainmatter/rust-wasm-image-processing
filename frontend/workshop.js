async function perform(what) {
  try {
    await what();
  } catch (error) {
    const errorFlash = document.querySelector("#error-flash");
    errorFlash.querySelector("#trace").innerHTML = error.stack;
    errorFlash.querySelector("#error-flash-close-button").addEventListener("click", () => {
      errorFlash.close();
    });
    errorFlash.showModal();
  }
}

window.triggerBackend = async function triggerBackend(url, params) {
  perform(async () => {
    throw new Error("fehl");
    let imageUrl = document.querySelector("#imageUrl").value;
    let imageElement = document.querySelector("#imageOutput");
    
    if (!imageUrl) {
      console.error("must specify image");
      return;
    }
    
    const queryString = new URLSearchParams({
      image_url: imageUrl,
      ...params,
    }).toString();
    let response = await fetch(`http://localhost:3001/${url}?${queryString}`);
    let blob = await response.blob();
    imageElement.src = URL.createObjectURL(blob);
  })
};

window.triggerWasm = async function triggerWasm(endpoint, ...params) {
  await wasm_bindgen();

  let imageUrl = document.querySelector("#imageUrl").value;
  let imageElement = document.querySelector("#imageOutput");

  if (!imageUrl) {
    console.error("must specify image");
    return;
  }

  const imageData = await loadImage(imageUrl);
  let output = wasm_bindgen[endpoint](imageData, ...params);
  const blob = new Blob([output], { type: "image/jpeg" });
  imageElement.src = URL.createObjectURL(blob);
};

window.triggerBackendExercise3 = async function triggerBackendExercise3() {
  let leftUrl = document.querySelector("#imageUrlLeft").value;
  let rightUrl = document.querySelector("#imageUrlRight").value;
  let imageElement = document.querySelector("#imageOutput");

  if (!leftUrl || !rightUrl) {
    console.error("must specify image");
    return;
  }

  const queryString = new URLSearchParams({
    left: leftUrl,
    right: rightUrl,
  }).toString();
  let response = await fetch(`http://localhost:3001/exercise_3?${queryString}`);
  let blob = await response.blob();
  imageElement.src = URL.createObjectURL(blob);
};

window.triggerWasmExercise3 = async function triggerWasmExercise3() {
  await wasm_bindgen();

  let leftUrl = document.querySelector("#imageUrlLeft").value;
  let rightUrl = document.querySelector("#imageUrlRight").value;
  let imageElement = document.querySelector("#imageOutput");

  if (!leftUrl || !rightUrl) {
    console.error("must specify image");
    return;
  }

  const [leftData, rightData] = await Promise.all([
    loadImage(leftUrl),
    loadImage(rightUrl),
  ]);
  let output = wasm_bindgen["exercise_3"](leftData, rightData);
  const blob = new Blob([output], { type: "image/jpeg" });
  imageElement.src = URL.createObjectURL(blob);
};

async function loadImage(imageUrl) {
  let response = await fetch(imageUrl);
  let buffer = await response.arrayBuffer();
  return new Uint8Array(buffer);
}
