// mod three;

// use three::*;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <main>
        <span class="fixed inset-0 z-0 bg-[repeating-linear-gradient(-45deg,_rgb(6,14,13)_0px,_rgb(6,14,13)_16px,_rgb(3,7,7)_16px,_rgb(3,7,7)_32px)]" />
            <div class="relative z-10">
               <div>
                    <nav class="relative mt-8 mx-12 rounded-xl border-2 border-mint bg-forest h-24 flex items-center gap-8">
                        <a href="/" class="text-forest rounded-lg ml-6 p-2 border-2 border-green bg-mint font-mono text-lg font-semibold transition-all duration-300 hover:border-mint hover:bg-green hover:text-mint hover:scale-105 hover:shadow-lg">Cloud Computing</a>
                        <a href="#historia" class="text-mint font-mono text-lg font-semibold transition-all duration-300 hover:text-mint/60 hover:scale-110">História</a>
                        <a href="#caracteristicas" class="text-mint font-mono text-lg font-semibold transition-all duration-300 hover:text-mint/60 hover:scale-110">Características</a>
                        <a href="#como-funciona" class="text-mint font-mono text-lg font-semibold transition-all duration-300 hover:text-mint/60 hover:scale-110">Como Funciona</a>
                        <a href="#modelos-cloud" class="text-mint font-mono text-lg font-semibold transition-all duration-300 hover:text-mint/60 hover:scale-110">Modelos de Cloud</a>
                        <a href="#implantacao" class="text-mint font-mono text-lg font-semibold transition-all duration-300 hover:text-mint/60 hover:scale-110">Modelos de implantação</a>
                    </nav>
                </div>
                <div id="main" class="m-12">
                    <h1 class="font-mono -ml-2 text-green font-semibold text-9xl leading-none">Cloud<br />Computing</h1>
                    <p class="text-mint my-2 pt-8 font-mono text-lg">Feito por Henrique Stelzer, Luís Alberto e Miguel Candido.</p>
                </div>

                <hr class="m-4 mx-12 mt-20 mb-12" />

                <div id="historia" class="m-12 flex">
                    <div class="w-1/3">
                        <h2 class="font-mono text-green font-semibold text-4xl leading-none">História</h2>
                        <p class="text-mint pt-8 font-mono text-justify text-lg indent-8">
                            A computação em nuvem (cloud computing) não foi criada por uma única pessoa, mas evoluiu de conceitos de "computação como utilidade" propostos por John McCarthy na década de 1960. No entanto, o termo moderno foi definido em 1997 pelo professor Ramnath Chellappa, com a Amazon Web Services (AWS) 2006 popularizando o modelo comercialmente.
                        </p>
                    </div>
                    <div>
                    </div>
                </div>
            </div>
        </main>
    }
}
