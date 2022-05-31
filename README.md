# RustBall

## Autorzy
- Daniel Mastalerz

## Opis
![HaxBall](https://i.ytimg.com/vi/JkAptaaFSrE/maxresdefault.jpg)
RustBall będzie połączeniem piłki nożnej i cymbergaja. Na boisku każdy gracz jest reprezentowany przez kulkę, celem gracza jest zdobycie bramek i końcowe wygranie meczu. Gra będzie wzorowana na grze [HaxBall](https://www.haxball.com) napisanej w języku Haxe.

## Funkcjonalność
- możliwość gry na jednym komputerze przez dwie osoby (z różnymi klawiszami do sterowania)
- różne typy boisk
- menu, zapisywanie stanu gry
- zapisywanie profilu gracza, statystyki
- możliwośc gry przez internet, z więcej niż jednym graczem w każdej drużynie, lobby do szukania meczu

## Propozycja podziału na części
W pierwszej części zaimplenetuję 4 pierwsze punkty, czyli wersja gry bez potrzeby łączenia się z siecią

W drugiej części punkt piąty - możliwość gry przez internet.

## Biblioteki
- Bevy
- coś do obsługi bazy danych (zapisywanie profili graczy i ich statystyk)
- coś do obsługi gry przez internet

## Postępy po pierwszej części
 - Zaimplementowana została rozgrywka między dwoma graczami, menu, oraz możliwość wyboru boiska. Punkty o zapisywaniu stanu gry oraz profilu gracza niestety pominięte przez brak czasu :(
 - Sterowanie: gracz czerwony – poruszanie się poprzez WASD, strzał spacją, gracz niebieski – poruszanie się poprzez strzałki, strzał prawym controlem.
 - Rozgrywka została zaimplementowana za pomocą biblioteki Bevy, poszczególne fragmenty gry obsługiwane są jako "systemy", które program wykonuje albo na początku programu, albo cały czas w sposób ciągły.
 - Została zaimplementowana obsługa kolizji (liczenie nowych wektorów prędkości za pomocą różnych wzorów).
 - W menu są trzy przyciski, możemy wyjść, zmienić boisko oraz rozpocząć grę. Po rozpoczęciu gry nie możemy już powrócić do menu, jeśli nie wyłączymy gry.
 - Sama rozgrywka trwa do momentu strzelenia przez któregoś z graczy trzech bramek. Jeśli tak się stanie, zwyciężca zostanie ogłoszony i gra rozpocznie się od nowa.