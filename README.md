# Grafy

## Autorzy
- Mikołaj Piróg
- Mikołaj Wasiak (@RudyMis)

## Opis

Program służy do edycji grafu oraz wizualizacji
algorytmów grafowych. 

Będziemy się wzorować na [tym edytorze](https://csacademy.com/app/graph_editor/) 

## Funkcjonalność

Edytowanie grafu
- Dodawanie i usuwanie wierzchołków 
- Łączenie wierzchołków krawędziami skierowanymi
lub nieskierowanymi
- Przesuwanie wierzchołków po okienku
- Zapisywanie jakiś wartości w wierzchołku 
do rozróżnienia/nazywania ich
- Automatyczną separacja wierzchołków
(jak w przykładzie)

Algorytmy grafowe
- Chcemy żeby było widać każdy krok grafu
- Kolorowanie wierzchołków w zależności czy są 
rozpatrzone/zakolejkowane/nierozpatrzone
- Dzielenie algorytmów na części gdy są skomplikowane
np silnie spójne da się podzielić na dfs i dfs odwroconymi krawędziami 

## Propozycje podziału na części 
Pierwsza to zrobienie edytora i napisani jakiegoś
prostego dfsa żeby zobaczyć czy struktury danych
pozwalają na algorytmowanie

Druga to dodanie algorytmów

## Biblioteki
Szukamy zarówno czegoś co da nam gui jak i czegoś 
na czym da się rysować kształty i je animować. Aktualnie stanęło na
[egui](https://docs.rs/egui) + [tetra](https://tetra.seventeencups.net/) 
