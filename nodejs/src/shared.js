const fs = require("fs");

async function readFilesToMemory(filesToRead) {
  const files = {};
  const stack = [...filesToRead];

  while (stack.length > 0) {
    const filePath = stack.pop();
    if (!filePath) {
      console.error("Skipping empty file");
      continue;
    }
    if (fs.lstatSync(filePath).isDirectory()) {
      const dirContents = fs.readdirSync(filePath);
      for (const file of dirContents) {
        stack.push(`${filePath}/${file}`);
      }
    } else {
      files[filePath] = await fs.promises.readFile(filePath, "utf-8");
    }
  }
  return files;
}

module.exports = { readFilesToMemory };