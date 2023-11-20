use leptos::{*};

/// Renders an image of old-timey railmen fighting with the title of the game and a centered gray box
/// under the title
#[component]
pub fn Layout(
    /// The elements to display within the gray box. Elements will be centered and column aligned.
    children: ChildrenFn
) -> impl IntoView {
    view! {
        <div class="absolute top-0 left-0 w-full h-full bg-cover bg-center" style="background-image: url('/assets/images/rail-riches.png');"></div>
        <div class="fixed top-0 left-0 w-full h-full bg-black bg-opacity-40 flex justify-center">
            <div class="flex flex-col items-center mt-20">
                <a class="text-center mb-10 text-8xl font-oldtimey underline-dashed text-blue-800" style="text-shadow: -1px -1px 0 #fff, 1px -1px 0 #fff, -1px 1px 0 #fff, 1px 1px 0 #fff;" href="/">Railway Riches</a>
                <div class="bg-gray-400 p-6 flex flex-col items-center w-80 justify-center space-y-3">
                    {children()}
                </div>
            </div>
        </div>
    }
}
   