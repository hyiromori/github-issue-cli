import { getCoreCommand } from "./lib/args.ts";
import { update } from "./command/update.ts";
import { create } from "./command/create.ts";
import { help } from "./command/help.ts";
import { debugLog } from "./lib/logger.ts";

const coreCommand = getCoreCommand();
debugLog(`Command: ${coreCommand}`);

switch (coreCommand) {
  case "create":
    await create();
    break;
  case "update":
    await update();
    break;
  default:
    help();
}
