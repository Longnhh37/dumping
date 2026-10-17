"use strict";

const DEFAULT_NAME = "Anonymous User";

const params = new URLSearchParams(window.location.search);
const username = (params.get("name") ?? "").trim() || DEFAULT_NAME;

document.getElementById("greeting").textContent = `Hello, ${username}!`;
