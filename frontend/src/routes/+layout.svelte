
<div style = "light">
    <slot />
</div>

<style lang="scss">
    @use "../styles/consts";

    :root {
        --background-color: #FFF1E5;
        --accent-color: darkred;
        --border-color: #9c938c;
        --font-color: #363636;
    }

    :global(body) {
        background-color: var(--background-color);
        font-family: consts.$serif;
        font-size: 23px;
        color: var(--font-color);
        max-width: 700px;
        line-height: 1.5;
        padding-left: 24px;
        padding-right: 24px;
        margin-top: 0px;
        margin-right: auto;
        margin-bottom: 20px;
        margin-left: auto;
        word-break: break-all
    }
    :global(h1) {
        font-family: consts.$sans;
        text-align: center;
        font-weight: 300;
        line-height: 1;
    }
    :global(h2) {
        font-weight: normal;
        font-size: 32px;
        padding-left: 20px;
    }
    :global(h3) {
        font-weight: normal;
        padding-left: 20px;
        font-size: 28px;
    }

    :global(div.sub-info) {
        font-family: consts.$mono;
        font-size: 16px;
    }

    :global(hr) {
        border-color: var(--border-color);
        border-bottom: none; 
        border-width: 2px;
    }

    :global(input) {
        background-color: var(--background-color);
        border-bottom-color: var(--border-color);
        border-bottom-width: 2px;
        border-left: none;
        border-right: none;
        border-top: none;
        outline: none;
        font-family: consts.$mono; 
        font-size: 20px;
    }

    :global(button) {
        font-family: consts.$mono;
        background-color: var(--background-color);
        padding: 10px;
        font-size: 15px;
        border-right: 0;
        border-left: 0;
        box-shadow: none;
        border-bottom: 2px solid var(--border-color);
        -webkit-transition: ease-out 0.4s;
        -moz-transition: ease-out 0.4s;
        transition: ease-out 0.4s;
    }
    :global(button:hover) {
        box-shadow: inset 0 0 0 50px #f5e8df;
    }
    :global(label) {
        font-family: consts.$mono;
    }
    :global(table, th, td) {
        font-family: consts.$sans;
        border-bottom: 1px solid #9c938c;
        border-collapse: collapse;
        padding: 6px;
        font-size: 17px;
    }

    :global(th) {
        font-family: consts.$mono;
        font-size: 20px;
        text-align: center;
        font-weight: normal;
    }

    :global(textarea) {
        font-family: consts.$mono;
        border-width: 2px;
        font-size: 15px;
    }


</style> 