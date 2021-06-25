// Import 'start' function from denops_std
import {main} from "https://deno.land/x/denops_std/mod.ts";
import * as pkg from '../../pkg'

// Call 'start' with async callback. The callback get 'vim' instance.
main(async ({vim}) => {
    // Initialize (loading wasm)
    await pkg.default();
    pkg.initialize();

    vim.register({
        async test1(_: unknown): Promise<void> {
            const test = await pkg.vim_test(vim);
            console.log(test);
        },
        async test2(_: unknown): Promise<void> {
            await pkg.vim_test2(vim);
        },

        async test3(_: unknown): Promise<void> {
            await pkg.vim_test3(vim);
        },
    });

    // Use 'vim.execute()' to execute Vim script
    await vim.execute(`
      command! DenopsRustTest1 call denops#notify("${vim.name}", "test1", [""])
      command! DenopsRustTest2 call denops#notify("${vim.name}", "test2", [""])
      command! DenopsRustTest3 call denops#notify("${vim.name}", "test3", [""])
      let g:denops_helloworld = "Global Hello Denops"
  `);

    console.log("denops-rust-examples.vim has loaded");
});