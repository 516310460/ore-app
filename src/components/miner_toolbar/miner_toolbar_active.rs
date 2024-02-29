use dioxus::prelude::*;
use dioxus_router::components::Link;
use ore::state::{Proof, Treasury};
#[cfg(feature = "desktop")]
use solana_account_decoder::parse_token::UiTokenAmount;
#[cfg(feature = "web")]
use solana_extra_wasm::account_decoder::parse_token::UiTokenAmount;

use crate::{
    components::{
        IsToolbarOpen, MinerPower, MinerStatus, OreIcon, StopButton, Tooltip, TooltipDirection,
    },
    gateway::AsyncResult,
    miner::Miner,
    route::Route,
};

use super::MinerStatusMessage;

// TODO Lifetime rewards
// TODO Lifetime hashes

// #[cfg(feature = "desktop")]
#[derive(Props, PartialEq)]
pub struct MinerToolbarActiveProps {
    pub treasury: AsyncResult<Treasury>,
    pub proof: AsyncResult<Proof>,
    pub ore_supply: AsyncResult<UiTokenAmount>,
    pub miner: UseState<Miner>,
}

#[component]
pub fn MinerToolbarActive(cx: Scope<MinerToolbarActiveProps>) -> Element {
    let timer = use_state(cx, || 0u64);
    let proof = cx.props.proof;
    let is_toolbar_open = use_shared_state::<IsToolbarOpen>(cx).unwrap();
    let miner_status_message = use_shared_state::<MinerStatusMessage>(cx).unwrap();

    let hash = match proof {
        AsyncResult::Ok(proof) => proof.hash.to_string(),
        _ => "–".to_string(),
    };

    let hash_abbr = if hash.len().gt(&16) {
        hash[0..16].to_string()
    } else {
        "–".to_string()
    };

    let claimable_rewards = match cx.props.proof {
        AsyncResult::Ok(proof) => {
            (proof.claimable_rewards as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64)
        }
        _ => 0f64,
    };

    let circulating_supply = match cx.props.treasury {
        AsyncResult::Ok(treasury) => {
            (treasury.total_claimed_rewards as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64)
        }
        _ => 0f64,
    };

    let ore_supply = match cx.props.ore_supply.clone() {
        AsyncResult::Ok(token_amount) => token_amount.ui_amount.unwrap().to_string(),
        AsyncResult::Loading => "-".to_string(),
        AsyncResult::Error(_err) => "Err".to_string(),
    };

    let reward_rate = match &cx.props.treasury {
        AsyncResult::Ok(treasury) => {
            (treasury.reward_rate as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64)
        }
        _ => 0f64,
    };

    use_effect(cx, &proof, |_| {
        timer.set(0);
        async move {}
    });

    let _n = use_future(cx, (), |_| {
        let timer = timer.clone();
        async move {
            loop {
                async_std::task::sleep(std::time::Duration::from_secs(1)).await;
                timer.set(*timer.current() + 1);
            }
        }
    });

    if is_toolbar_open.read().0 {
        render! {
            div {
                class: "flex flex-col grow gap-4 px-4 sm:px-8 py-8",
                div {
                    class: "flex flex-row w-full justify-between",
                    h2 {
                        class: "text-2xl text-white font-bold",
                        "Mining"
                    }
                    div {
                        class: "flex flex-row gap-4",
                        StopButton {
                            miner: cx.props.miner.clone()
                        }
                    }
                }
                div {
                    class: "grid grid-cols-3 grid-rows-2 gap-y-8",
                    // MinerDataOre {
                    //     title: "Your rewards",
                    //     tooltip: "The amount of Ore you have mined and may claim.",
                    //     amount: claimable_rewards.to_string(),
                    //     claimable_rewards: claimable_rewards
                    // }
                    MinerDataOre {
                        title: "Reward rate",
                        tooltip: "The amount of Ore you are earning per valid hash.",
                        amount: reward_rate.to_string()
                    }
                    MinerDataOre {
                        title: "Circulating supply",
                        tooltip: "The total amount of Ore that has ever been claimed.",
                        amount: circulating_supply.to_string()
                    }
                    MinerDataOre {
                        title: "Total supply",
                        tooltip: "The total amount of Ore that has ever been mined.",
                        amount: ore_supply
                    }
                }
                MinerPower {}
            }
        }
    } else {
        render! {
            div {
                class: "flex flex-row w-full justify-between my-auto px-4 sm:px-8",
                div {
                    class: "flex flex-row gap-2 my-auto",
                    ActivityIndicator {}
                    p {
                        class: "font-semibold text-white",
                        "Mining"
                    }
                }
                StopButton {
                    miner: cx.props.miner.clone()
                }
            }
        }
    }
}

#[component]
pub fn MinerDataOre<'a>(cx: Scope, title: &'a str, tooltip: &'a str, amount: String) -> Element {
    let container_class = "flex flex-col gap-0 shrink h-min";
    let header_container_class = "flex flex-row justify-start gap-1.5";
    let header_class = "font-medium text-xs z-0 text-nowrap opacity-80";
    let value_class = "font-medium text-white h-8";
    render! {
        div {
            class: "{container_class} w-full",
            div {
                class: "{header_container_class}",
                p {
                    class: "{header_class}",
                    "{title}"
                }
                Tooltip {
                    text: "{tooltip}",
                    direction: TooltipDirection::Right
                }
            }
            div {
                class: "flex flex-row gap-8",
                p {
                    class: "{value_class} flex flex-row flex-nowrap text-nowrap place-items-baseline",
                    OreIcon {
                        class: "w-4 h-4 my-auto",
                    }
                    span {
                        class: "ml-1.5 my-auto",
                        "{amount}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn ActivityIndicator(cx: Scope) -> Element {
    // let class = cx.props.class.unwrap_or("");
    render! {
        span {
            class: "relative flex h-3 w-3 justify center my-auto",
            span {
                class: "animate-ping absolute inline-flex h-full w-full rounded-full opacity-75 bg-white",
                " "
            }
            span {
                class: "relative inline-flex rounded-full h-2 w-2 my-auto mx-auto bg-white"
            }
        }
    }
}
