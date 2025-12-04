use azumi::html;

/// Lesson 9: Feature Composition
///
/// Combining multiple Azumi features
#[azumi::component]
pub fn feature_showcase() -> impl azumi::Component {
    html! {
        <div .showcase>
            <div .section>
                <h2 #feature_showcase_name>"Feature Composition"</h2>
                <span .active_badge>"ACTIVE"</span>
                <div .feature_list>
                    <div .feature_item>"Feature 1"</div>
                    <div .feature_item>"Feature 2"</div>
                    <div .feature_item>"Feature 3"</div>
                </div>
            </div>
        </div>
    }   
}