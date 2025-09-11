+++
title = "Uke 36"
date = 2025-09-07
author = "simsine"
+++

> Timer denne uka: x
> 
> Timer totalt: x

# Onsdag - 

## Error handling i Axum
I dag startet jeg å implementere feilhåndtering i Axum applikasjonen, ettersom dette ikke var implementert ennå manglet siden en egen error respons side og returnerte bare en generisk browser error. Heldigvis oppgir Axum flere kode-eksempler på blant annet error handling i [GitHub repoet deres](https://github.com/tokio-rs/axum/tree/main/examples). Her vises det blant annet hvordan man kan gjøre om route handler funksjoner til å returnere en Result type. Det oppgir også hvordan man kan lage error typer man oppgir i Result typen som så kan implementere en metode for å konvertere error typen til en Response med en http respons kode og en respons body.

```rust
#[derive(Deserialize, Debug)]
struct ErrorParams {
   should_bail: bool,
}

fn error_prone_function(should_bail: bool) -> Result<(), anyhow::Error> {
   if should_bail {
       anyhow::bail!("Something has gone wrong :(")
   } else {
       Ok(())
   }
}


async fn error_prone_handler(
   Query(params): Query<ErrorParams>,
) -> Result<Response, AppError> {
   error_prone_function(params.should_bail)?;


   Ok(maud!(
       h1 {"Everything is a-ok!"}
   ).render().into_response())
}
```

```rust
pub struct AppError(anyhow::Error);

impl<E> From<E> for AppError
where
   E: Into<anyhow::Error>,
{
   fn from(err: E) -> Self {
       Self(err.into())
   }
}


impl IntoResponse for AppError {
   fn into_response(self) -> Response {
       (StatusCode::INTERNAL_SERVER_ERROR, maud!{
           h1 {(StatusCode::NOT_FOUND.as_u16())}
           p {(self.0.to_string())}
       }.render()).into_response()
   }
}
```

Her introduserer vi også en ganske fin syntaks som tillater oss å kalle funksjoner på måten error_prone_function()?. Dette betyr at funksjonen returnerer et resultat som gjøres om til et Result som returneres av handleren. Her fungerer ? operatoren som en early return.

Jeg la også til en fallback handler for routeren.

```rust
pub async fn error_404() -> impl IntoResponse {
   (StatusCode::NOT_FOUND, maud!(
       h1 {"404"}
       p {"Page not found"}
   ).render().into_response())
}

let app = Router::new()
    .route("/maybe_error", get(error_prone_handler))
    .fallback(error_404);
```

# Torsdag - 5,5 timer - 10:30-16:00
I dag knotet jeg en del med CSS styling. Noe av grunnen til det var at jeg skulle prøve å endre farge på SVG ikonene på siden basert på kontekst. Dette gikk ikke helt som forventet og det gikk en del tid til googling for å prøve å finne en løsning. Måten vi importerer SVG’ene på er å bruke en *img* tag som så refererer til en SVG fil, noe som viser seg å føre til noen begrensninger for å manipulere det med css. Jeg har imidlertid en mulig løsning på dette, men det må jeg komme tilbake til senere. Mot slutten av dagen blir jeg invitert med på en såkalt Back to work middag med de andre ansatte. Vi drar på restauranten SJØ Restaurant og Vinbar og blir servert en 3-retters middag og vin til drikke. Det skal nevnes at biffen deres er veldig god.

# Fredag - 3 timer - 12:00-15:00 
Fikk problemer med bluetooth på laptopen, så det tok en drøy time å fikse. Etter jeg fikk fiksa bluetooth og kunne bruke keyboardet igjen begynte jeg å se på integrering av Bacon-LS i VSCode. Dagen var generelt preget av gårsdagens affære og at folk var klare for helgen og andre startet på en sen sommerferie.
