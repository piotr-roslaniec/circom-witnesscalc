const fs = require("fs");
const path = require("path");
const { build_circuit, BuildCircuitArgs } = require("witnesscalc");

function parseArgs() {
  const args = process.argv.slice(2);
  let i = 0;
  let circuitFile = null;
  let graphFile = null;
  let linkLibraries = [];
  let inputsFile = null;
  let printUnoptimized = false;
  let printDebug = false;

  function usage(errMsg) {
    console.error(errMsg);
    console.error(
      `Usage: ${process.argv[0]} <circuit_file> <graph_file> [-l <link_library>]* [-i <inputs_file.json>] [-print-unoptimized] [-v]`,
    );
    process.exit(1);
  }

  while (i < args.length) {
    if (args[i] === "-l") {
      i += 1;
      if (i >= args.length) {
        usage("missing argument for -l");
      }
      linkLibraries.push(path.resolve(args[i]));
    } else if (args[i].startsWith("-l")) {
      linkLibraries.push(path.resolve(args[i].slice(2)));
    } else if (args[i] === "-i") {
      i += 1;
      if (i >= args.length) {
        usage("missing argument for -i");
      }
      if (!inputsFile) {
        inputsFile = args[i];
      } else {
        usage("multiple inputs files");
      }
    } else if (args[i].startsWith("-i")) {
      if (!inputsFile) {
        inputsFile = args[i].slice(2);
      } else {
        usage("multiple inputs files");
      }
    } else if (args[i] === "-print-unoptimized") {
      printUnoptimized = true;
    } else if (args[i] === "-v") {
      printDebug = true;
    } else if (args[i].startsWith("-")) {
      usage(`unknown argument: ${args[i]}`);
    } else if (!circuitFile) {
      circuitFile = path.resolve(args[i]);
    } else if (!graphFile) {
      graphFile = path.resolve(args[i]);
    } else {
      usage(`unexpected argument: ${args[i]}`);
    }
    i += 1;
  }

  if (!circuitFile) usage("missing circuit file");
  if (!graphFile) usage("missing graph file");

  // Resolve and check if files exist
  if (!fs.existsSync(circuitFile)) {
    usage(`circuit file does not exist: ${circuitFile}`);
  }
  if (!fs.existsSync(graphFile)) {
    usage(`graph file does not exist: ${graphFile}`);
  }
  if (inputsFile) {
    inputsFile = path.resolve(inputsFile);
    if (!fs.existsSync(inputsFile)) {
      usage(`inputs file does not exist: ${inputsFile}`);
    }
  }
  linkLibraries = linkLibraries.map((lib) => {
    if (!fs.existsSync(lib)) {
      usage(`link library does not exist: ${lib}`);
    }
    return lib;
  });

  return new BuildCircuitArgs(
    circuitFile,
    inputsFile,
    graphFile,
    linkLibraries,
    printUnoptimized,
    printDebug,
  );
}

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


async function main() {
  const args = parseArgs();

  const argsObj = args.to_js_object();
  console.log({ argsObj });

  const filesToLoad = [
    argsObj.circuit_file,
    argsObj.input_file,
    argsObj.graph_file,
    ...argsObj.link_libraries,
  ];
  const filesMap = await readFilesToMemory(filesToLoad);
  console.log(`Loaded ${Object.keys(filesMap).length} files to memory`);

  const version = "2.1.9";
  const bytes = build_circuit(args, version, filesMap);
  const buffer = Buffer.from(bytes);
  console.log(`circuit graph size: ${buffer.length} bytes`);
  fs.writeFileSync(argsObj.graph_file, buffer);
  console.log(`circuit graph saved to file: ${argsObj.graph_file}`);
}

main();
