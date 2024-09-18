const fs = require("fs");
const { calc_witness } = require("witnesscalc");

function parseArgs() {
  const args = process.argv.slice(2);
  if (args.length !== 3) {
    console.error(
      `Usage: ${process.argv[0]} <graph.bin> <inputs.json> <witness.wtns>`,
    );
    process.exit(1);
  }

  const [graphFile, inputsFile, witnessFile] = args;

  if (!fs.existsSync(graphFile)) {
    console.error(`Graph file not found: ${graphFile}`);
    process.exit(1);
  }

  if (!fs.existsSync(inputsFile)) {
    console.error(`Inputs file not found: ${inputsFile}`);
    process.exit(1);
  }

  return { graphFile, inputsFile, witnessFile };
}

async function main() {
  const { graphFile, inputsFile, witnessFile } = parseArgs();

  const inputsData = await fs.promises.readFile(inputsFile, "utf-8");
  const graphData = await fs.promises.readFile(graphFile);
  const graphDataUint8Array = new Uint8Array(graphData.buffer, graphData.byteOffset, graphData.byteLength);

  const wtnsBytes = calc_witness(inputsData, graphDataUint8Array);

  await fs.promises.writeFile(witnessFile, wtnsBytes);
  console.log(`Witness saved to ${witnessFile}`);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
