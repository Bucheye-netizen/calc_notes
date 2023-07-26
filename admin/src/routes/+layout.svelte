<slot /> 

<style lang="scss"> 
    @use "../styles/themes";
    @use "../styles/consts";
    $color: #0cf;

    :global(body) {
        @include themes.light;
        font-family: consts.$sans;
        font-size: 20px;
        max-width: 700px;
        line-height: 1.5;
        padding-left: 18px;
        padding-right: 24px;
        margin-top: 0px;
        margin-right: auto;
        margin-bottom: 20px;
        margin-left: auto;
        word-break: break-all
    }
    :global(h1) {
        font-family: consts.$mono;
        text-align: center;
        font-weight: 300;
        font-size: 35px;
        line-height: 1;
    }
    :global(h2) {
        font-family: consts.$mono;
        font-weight: normal;
        font-size: 32px;
        padding-left: 20px;
    }
    :global(h3) {
        font-family: consts.$mono;
        font-weight: normal;
        padding-left: 20px;
        font-size: 28px;
    }
    :global(div.sub-info) {
        font-family: 'Courier Prime';
        font-size: 16px;
    }

    :global(input[type=text], input[type=password]) {
        @include themes.light;
        font-family: consts.$mono; 
        border-top: 0;
        border-right: 0;
        border-left: 0;
        font-size: 20px;
    }

    :global(input[type=text]:focus, input[type=password]:focus) {
        outline: none;
    }

    :global(button) {
        @include themes.light;
        font-family: consts.$mono;
        padding: 10px;
        font-size: 15px;
        border-right: 0;
        border-left: 0;
        box-shadow: none;
        border-bottom: 2px solid #9c938c;
        border-top: 2px solid #9c938c;
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
        @include themes.light;
        border-bottom: 1px solid #9c938c;
        border-collapse: collapse;
        padding: 6px;
    }
    :global(th) {
        font-family: consts.$mono;
        text-align: center;
        font-weight: normal;
    }
</style> 