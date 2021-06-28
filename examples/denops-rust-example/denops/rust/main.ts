// Import 'start' function from denops_std
import { Denops } from "https://deno.land/x/denops_std/mod.ts";
import { execute } from "https://deno.land/x/denops_std/helper/mod.ts";
import * as pkg from "../../pkg/index.js";

// Call 'start' with async callback. The callback get 'vim' instance.
export async function main(denops: Denops) {
  // Initialize (loading wasm)
  await pkg.default();
  // pkg.initialize();
  denops.dispatcher = {
    async test1(): Promise<void> {
      const test = await pkg.vim_test(denops);
      console.log(test);
    },
    async test2(_: unknown): Promise<void> {
      await pkg.vim_test2(denops);
    },

    async test3(_: unknown): Promise<void> {
      await pkg.vim_test3(denops);
    },
  };
  // Use 'execute()' to execute multiline Vim script
  await execute(
    denops,
    `
      command! DenopsRustTest1 call denops#notify("${denops.name}", "test1", [""])
      command! DenopsRustTest2 call denops#notify("${denops.name}", "test2", [""])
      command! DenopsRustTest3 call denops#notify("${denops.name}", "test3", [""])
    `,
  );
  console.log("denops-rust-examples.vim has loaded");
}
