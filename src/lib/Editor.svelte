<script lang="ts">
  import { editorContent, appMode } from './store';

  let content: string = '';
  let modeVal: string;

  editorContent.subscribe(v => content = v);
  appMode.subscribe(v => modeVal = v);

  function handleInput(e: Event) {
    const textarea = e.target as HTMLTextAreaElement;
    editorContent.set(textarea.value);
  }

  function getPlaceholder(): string {
    if (modeVal === 'correction') {
      return 'Write your diary in the target language...';
    }
    return '用繁體中文寫日記，AI 會翻譯成目標語言...';
  }
</script>

<div class="editor-wrapper">
  <textarea
    class="editor-textarea"
    value={content}
    oninput={handleInput}
    placeholder={getPlaceholder()}
    spellcheck="false"
  ></textarea>
</div>

<style>
  .editor-wrapper {
    flex: 1;
    overflow: hidden;
    display: flex;
  }

  .editor-textarea {
    width: 100%;
    height: 100%;
    border: none;
    background: transparent;
    resize: none;
    padding: 16px;
    line-height: 1.8;
    font-size: 15px;
    outline: none;
    color: var(--text-primary);
    font-family: inherit;
  }

  .editor-textarea::placeholder {
    color: var(--text-muted);
  }
</style>
