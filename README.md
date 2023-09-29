<div id="top"></div>

<div align="center">
  <!-- Keep one empty line -->

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
</div>

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <!-- <a href="https://github.com/lenra-io/template-hello-world-node12">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a> -->

<h1>App Lib for Rust based <a href="https://www.lenra.io">Lenra</a> projects</h1>

  <p>
    This lib integrates all the elements the app needs in order to only keep the views, listeners and resources in the app project.
  </p>

[Report Bug](https://github.com/lenra-io/app-lib-rust/issues)
·
[Request Feature](https://github.com/lenra-io/app-lib-rust/issues)
</div>


<!-- USAGE EXAMPLES -->
## Usage

To add it to your Lenra app project:
```console
cargo add lenra-app@~1.0.0-beta.0
```

### Lenra API calls

To call the Lenra API from a listener, just use the Api instance in the parameter of your listener function.

You can then call the data API like that to create a document:
```rust
#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct CustomType {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub value: String,
}

impl Doc for CustomType {
    fn id(&self) -> Option<String> {
        match &self.id {
            Some(x) => Some(x.clone()),
            None => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct CustomProps {
    value: String,
}
// Use the props! macro to generate the props struct automatically
props!(CustomProps);

fn my_listener(params: ListenerParams<CustomProps, Value>) -> Result<()> {
    let my_doc: CustomType = params.api.data.create_doc(
        COUNTER_COLLECTION,
        CustomProps {
            value: CustomProps.value,
            ..Default::default()
        },
    )?;
    Ok(())
}
```

<p align="right">(<a href="#top">back to top</a>)</p>


<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please open an issue with the tag "enhancement" or "bug".
Don't forget to give the project a star! Thanks again!

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the **MIT** License. See [LICENSE](./LICENSE) for more information.

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Lenra - [@lenra_dev](https://twitter.com/lenra_dev) - contact@lenra.io

Project Link: [https://github.com/lenra-io/app-lib-rust](https://github.com/lenra-io/app-lib-rust)

<p align="right">(<a href="#top">back to top</a>)</p>


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/lenra-io/app-lib-rust.svg?style=for-the-badge
[contributors-url]: https://github.com/lenra-io/app-lib-rust/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/lenra-io/app-lib-rust.svg?style=for-the-badge
[forks-url]: https://github.com/lenra-io/app-lib-rust/network/members
[stars-shield]: https://img.shields.io/github/stars/lenra-io/app-lib-rust.svg?style=for-the-badge
[stars-url]: https://github.com/lenra-io/app-lib-rust/stargazers
[issues-shield]: https://img.shields.io/github/issues/lenra-io/app-lib-rust.svg?style=for-the-badge
[issues-url]: https://github.com/lenra-io/app-lib-rust/issues
[license-shield]: https://img.shields.io/github/license/lenra-io/app-lib-rust.svg?style=for-the-badge
[license-url]: https://github.com/lenra-io/app-lib-rust/blob/master/LICENSE.txt
