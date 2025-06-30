// deno-lint-ignore-file
import { expandGlob } from "https://deno.land/std@0.224.0/fs/mod.ts";
import { join } from "https://deno.land/std@0.224.0/path/mod.ts";

function parseDesktopFile(content) {
  const lines = content.split("\n");
  let name = "";
  let exec = "";
  for (const line of lines) {
    if (line.startsWith("Name=")) {
      name = line.substring("Name=".length).trim();
    }
    if (line.startsWith("Exec=")) {
      exec = line.substring("Exec=".length).trim().split(" ")[0];
    }
  }
  if (name && exec) {
    return { name, exec };
  }
  return null;
}

async function findLinuxApps() {
  const desktopPaths = [
    "/usr/share/applications",
    "/usr/local/share/applications",
    join(Deno.env.get("HOME") || "", ".local/share/applications"),
  ];

  const apps = [];

  for (const path of desktopPaths) {
    try {
      for await (const entry of expandGlob(join(path, "*.desktop"))) {
        if (entry.isFile) {
          const content = await Deno.readTextFile(entry.path);
          const parsed = parseDesktopFile(content);
          if (parsed) {
            apps.push(parsed);
          }
        }
      }
    } catch (_) {
      continue;
    }
  }

  return apps;
}

export default async (input) => {
    const apps = await findLinuxApps();
    let res = [];
    for(let app in apps){
        res.push(item(apps[app].name, "Open app", "open-app.js:" + apps[app].exec));
    }
    return res;
}