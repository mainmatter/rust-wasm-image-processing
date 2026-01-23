window.triggerBackend = async function triggerBackend(url, params) {
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

async function loadImage(imageUrl) {
  let response = await fetch(imageUrl);
  let buffer = await response.arrayBuffer();
  return new Uint8Array(buffer);
}
