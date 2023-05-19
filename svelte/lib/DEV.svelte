<script lang="ts">
  import { invoke, type InvokeArgs } from "@tauri-apps/api/tauri";

  export let cmd: string;
  export let args: InvokeArgs = {};

  let response_success = "";
  let response_errors = "";

  if (cmd !== "")
    invoke(cmd, args)
      .then((response) => {
        console.log("TAURI INVOKE RESPONSE:\n", response, "\n");

        switch (typeof response) {
          case "string":
            response_success = response;
            console.log(`STRING: |${response}|`);
            break;

          case "number":
            response_success = response.toString();
            break;

          case "boolean":
            response_success = response ? "true" : "false";
            break;

          case "object":
            if (response === undefined || response === null) {
              break;
            }

            response_success = JSON.stringify(response, null, 2);

            break;

          default:
            break;
        }
      })
      .catch((err) => {
        console.log("ERROR:\n", err);
        if (err instanceof Error) {
          response_errors = err.message;
        }
      });

  $: lines_success = to_lines(response_success);
  $: lines_errors = to_lines(response_errors);

  function to_lines(str: string): { indent: string; content: string }[] {
    return str
      .split("\n")
      .filter((line) => line.length > 0)
      .map((str) => {
        let indent = 0;
        while (str.startsWith(" ")) {
          str = str.slice(1);
          indent++;
        }

        return {
          indent: `${indent * 0}rem`,
          content: str,
        };
      });
  }
</script>

<div>
  <h2>
    <em class="title">Tauri Command:</em>
    <em class="cmd">{cmd}</em>
  </h2>
  <!-- <h3 class="cmd">{cmd}</h3> -->

  <h3 class="success">Success Response:</h3>
  <div class="code-block">
    {#if lines_success.length === 0}
      <p class="line">NA</p>
    {:else}
      {#each lines_success as { indent, content }}
        <p class="line" style:padding-left={indent}>{content}</p>
      {/each}
    {/if}
  </div>

  <h3 class="error">Error Response:</h3>
  <div class="code-block">
    {#if lines_errors.length === 0}
      <p class="line">NA</p>
    {/if}
  </div>
</div>

<style>
  h2 {
    margin: 0.75rem auto 0.25rem;
    padding: 0;
  }
  h2 .title {
    color: violet;
    font-size: 1.3rem;
    font-style: normal;
  }
  h2 .cmd {
    color: darkturquoise;
  }

  h3 {
    margin: 1rem 0.25rem auto;
    padding: 0 1rem;
    text-align: left;
  }
  h3.success {
    color: rgb(113, 225, 1);
  }
  h3.error {
    color: rgb(220, 25, 64);
  }

  .code-block {
    background-color: #000;
    padding: 0.5rem 1rem;
    margin: 0.25rem 1rem;
    text-align: left;
  }
  .line {
    margin: 0;
    padding: 0;
  }
</style>
