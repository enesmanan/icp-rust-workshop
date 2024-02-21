import { qrcode_backend } from "../../declarations/qrcode_backend";

document.getElementById("generate").onclick = onGenerateButtonClick;

async function onGenerateButtonClick(event) {
  event.preventDefault();

  const buttonElement = event.target;
  const messageElement = document.getElementById("message");
  const imageElement = document.getElementById("qrcode");
  const linkElement = document.getElementById("download");

  buttonElement.disabled = true;
  messageElement.innerText = "Working...";
  imageElement.src = "";
  imageElement.width = 0;
  linkElement.href = "";

  try {
    const text = document.getElementById("text").value.toString();
    const options = {
      add_logo: document.getElementById("logo").checked,
      add_gradient: document.getElementById("gradient").checked,
      add_transparency: [document.getElementById("transparent").checked],
    }

    let result;
    if (document.getElementById("consensus").checked) {
      result = await qrcode_backend.qrcode(text, options);
    } else {
      result = await qrcode_backend.qrcode_query(text, options);
    }

    if ("Err" in result) {
      throw result.Err;
    }

    const image = result.Image;
    const blob = new Blob([image], { type: "image/png" });
    const url = await convertToDataUrl(blob);

    messageElement.innerText = "Here you go:";
    imageElement.src = url;
    imageElement.width = document.getElementById("text").clientWidth;
    linkElement.href = url;
  } catch (err) {
    messageElement.innerText = "Failed to generate QR code: " + err.toString();
  }

  buttonElement.disabled = false;
  return false;

}


function convertToDataUrl(blob) {
  return new Promise((resolve, _) => {
    const fileReader = new FileReader();
    fileReader.readAsDataURL(blob);
    fileReader.onloadend = function () {
      resolve(fileReader.result);
    }
  });
}
