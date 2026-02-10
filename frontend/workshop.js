async function perform(what) {
  const loading = document.querySelector("#loading");
  loading.style.display = "block";
  const timingInfo = document.querySelector("#timing-info");
  timingInfo.innerHTML = "";
  let imageElement = document.querySelector("#imageOutput");
  imageElement.src = "";

  try {
    const now = performance.now();
    const imageSrc = await what();
    imageElement.src = imageSrc;
    const later = performance.now();
    const ms = Math.round(later - now);

    timingInfo.innerHTML = `(took ${ms}ms)`;
  } catch (error) {
    console.error(error);
    const errorFlash = document.querySelector("#error-flash");
    errorFlash.querySelector("#trace").innerHTML = `${error} \n ${error.stack}`;
    errorFlash
      .querySelector("#error-flash-close-button")
      .addEventListener("click", () => {
        errorFlash.close();
      });
    errorFlash.showModal();
  } finally {
    loading.style.display = "none";
  }
}

window.triggerBackend = async function triggerBackend(url, params = {}) {
  perform(async () => {
    let imageUrl = document.querySelector("#imageUrl").value;
    if (!imageUrl) {
      throw new Error("image url cannot be blank");
    }

    const query = new URLSearchParams({
      image_url: imageUrl,
    });

    for (const [key, value] of params) {
      query.append(key, value);
    }

    let response = await fetch(
      `http://localhost:3001/${url}?${query.toString()}`,
    );
    let blob = await response.blob();
    return URL.createObjectURL(blob);
  });
};

window.triggerWasm = async function triggerWasm(endpoint, ...params) {
  perform(async () => {
    await wasm_bindgen();

    let imageUrl = document.querySelector("#imageUrl").value;
    if (!imageUrl) {
      throw new Error("image url cannot be blank");
    }

    const imageData = await loadImage(imageUrl);
    let output = wasm_bindgen[endpoint](imageData, ...params);
    const blob = new Blob([output], { type: "image/jpeg" });
    return URL.createObjectURL(blob);
  });
};

window.triggerBackendExercise3 = async function triggerBackendExercise3() {
  perform(async () => {
    let leftUrl = document.querySelector("#imageUrlLeft").value;
    let rightUrl = document.querySelector("#imageUrlRight").value;
    if (!leftUrl || !rightUrl) {
      throw new Error("image url cannot be blank");
    }

    const queryString = new URLSearchParams({
      left: leftUrl,
      right: rightUrl,
    }).toString();
    let response = await fetch(
      `http://localhost:3001/exercise_3?${queryString}`,
    );
    let blob = await response.blob();
    return URL.createObjectURL(blob);
  });
};

window.triggerWasmExercise3 = async function triggerWasmExercise3() {
  perform(async () => {
    await wasm_bindgen();

    let leftUrl = document.querySelector("#imageUrlLeft").value;
    let rightUrl = document.querySelector("#imageUrlRight").value;
    if (!leftUrl || !rightUrl) {
      throw new Error("image url cannot be blank");
    }

    const [leftData, rightData] = await Promise.all([
      loadImage(leftUrl),
      loadImage(rightUrl),
    ]);
    let output = wasm_bindgen["exercise_3"](leftData, rightData);
    const blob = new Blob([output], { type: "image/jpeg" });
    return URL.createObjectURL(blob);
  });
};

async function loadImage(imageUrl) {
  let proxiedImageUrl = `http://localhost:3001/image_proxy?url=${encodeURI(imageUrl)}`;
  let response = await fetch(proxiedImageUrl);
  let buffer = await response.arrayBuffer();
  return new Uint8Array(buffer);
}

// ----- IMAGE COMPARISON SLIDER (https://codepen.io/stanko/pen/myddXKm) ----- //

const compareView = document.querySelector(".workshop-output--compare");
const compareViewSlider = document.querySelector(".workshop-output--compare input");

if (compareView && compareViewSlider) {
  compareViewSlider.addEventListener("input", () => {
    compareView.style.setProperty("--mask-width", `${compareViewSlider.value}%`);
  });
  
  compareView.style.setProperty("--mask-width", `${compareViewSlider.value}%`);
}

const imageUrl = document.querySelector("#imageUrl");
const inputImage = document.querySelector("#imageInput");
const outputImage = document.querySelector("#imageOutput");

if (imageUrl) {
  imageUrl.addEventListener("input", () => {
    if (inputImage) {
      inputImage.src = imageUrl.value;
    }
    outputImage.src = "";
  });
  if (inputImage) {
    inputImage.src = imageUrl.value;
  }
  outputImage.src = "";
}
