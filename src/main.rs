mod markov;

use markov::Model;

fn main() {
    let mut mk = Model::new_prose();
    mk.train_paragraph("French (français [fʁɑ̃sɛ] or langue française [lɑ̃ɡ fʁɑ̃sɛːz]) is a Romance language of the Indo-European family. It descended from the Vulgar Latin of the Roman Empire, as did all Romance languages. French evolved from Gallo-Romance, the Latin spoken in Gaul, and more specifically in Northern Gaul. Its closest relatives are the other langues d'oïl—languages historically spoken in northern France and in southern Belgium, which French (Francien) largely supplanted. French was also influenced by native Celtic languages of Northern Roman Gaul like Gallia Belgica and by the (Germanic) Frankish language of the post-Roman Frankish invaders. Today, owing to France's past overseas expansion, there are numerous French-based creole languages, most notably Haitian Creole. A French-speaking person or nation may be referred to as Francophone in both English and French.");
    mk.train_paragraph("French is an official language in 29 countries across multiple continents, most of which are members of the Organisation internationale de la Francophonie (OIF), the community of 84 countries which share the official use or teaching of French. French is also one of six official languages used in the United Nations. It is spoken as a first language (in descending order of the number of speakers) in France; Canada (especially in the provinces of Quebec, Ontario, and New Brunswick, as well as other Francophone regions); Belgium (Wallonia and the Brussels-Capital Region); western Switzerland (specifically the cantons forming the Romandy region); parts of Luxembourg; parts of the United States (the states of Louisiana, Maine, New Hampshire and Vermont); Monaco; the Aosta Valley region of Italy; and various communities elsewhere.");
    mk.train_paragraph("In 2015, approximately 40% of the francophone population (including L2 and partial speakers) lived in Europe, 36% in sub-Saharan Africa and the Indian Ocean, 15% in North Africa and the Middle East, 8% in the Americas, and 1% in Asia and Oceania. French is the second most widely spoken mother tongue in the European Union. Of Europeans who speak other languages natively, approximately one-fifth are able to speak French as a second language. French is the second most taught foreign language in the EU. All institutions of the EU use French as a working language along with English and German; in certain institutions, French is the sole working language (e.g. at the Court of Justice of the European Union). French is also the 18th most natively spoken language in the world, fifth most spoken language by total number of speakers and the second or third most studied language worldwide (with about 120 million learners as of 2017). As a result of French and Belgian colonialism from the 16th century onward, French was introduced to new territories in the Americas, Africa and Asia. Most second-language speakers reside in Francophone Africa, in particular Gabon, Algeria, Morocco, Tunisia, Mauritius, Senegal and Ivory Coast.");
    mk.train_paragraph("French is estimated to have about 76 million native speakers; about 235 million daily, fluent speakers; and another 77–110 million secondary speakers who speak it as a second language to varying degrees of proficiency, mainly in Africa. According to the OIF, approximately 321 million people worldwide are \"able, to, speak the language\", without specifying the criteria for this estimation or whom it encompasses. According to a demographic projection led by the Université Laval and the Réseau Démographie de l'Agence universitaire de la Francophonie, the total number of French speakers will reach approximately 500 million in 2025 and 650 million by 2050. OIF estimates 700 million by 2050, 80% of whom will be in Africa.");
    mk.train_paragraph("French has a long history as an international language of literature and scientific standards and is a primary or second language of many international organisations including the United Nations, the European Union, the North Atlantic Treaty Organization, the World Trade Organization, the International Olympic Committee, and the International Committee of the Red Cross. In 2011, Bloomberg Businessweek ranked French the third most useful language for business, after English and Standard Mandarin Chinese.");
    mk.train_paragraph("French is a Romance language (meaning that it is descended primarily from Vulgar Latin) that evolved out of the Gallo-Romance dialects spoken in northern France. The language's early forms include Old French and Middle French.");
    mk.train_paragraph("Due to Roman rule, Latin was gradually adopted by the inhabitants of Gaul, and as the language was learned by the common people it developed a distinct local character, with grammatical differences from Latin as spoken elsewhere, some of which being attested on graffiti. This local variety evolved into the Gallo-Romance tongues, which include French and its closest relatives, such as Arpitan.");
    mk.train_paragraph("The evolution of Latin in Gaul was shaped by its coexistence for over half a millennium beside the native Celtic Gaulish language, which did not go extinct until the late sixth century, long after the fall of the Western Roman Empire. The population remained 90% indigenous in origin; the Romanizing class were the local native elite (not Roman settlers), whose children learned Latin in Roman schools. At the time of the collapse of the Empire, this local elite had been slowly abandoning Gaulish entirely, but the rural and lower class populations remained Gaulish speakers who could sometimes also speak Latin or Greek. The final language shift from Gaulish to Vulgar Latin among rural and lower class populations occurred later, when both they and the incoming Frankish ruler/military class adopted the Gallo-Roman Vulgar Latin speech of the urban intellectual elite.");
    mk.train_paragraph("The Gaulish language likely survived into the sixth century in France despite considerable Romanization. Coexisting with Latin, Gaulish helped shape the Vulgar Latin dialects that developed into French contributing loanwords and calques (including oui, the word for \"yes\"), sound changes shaped by Gaulish influence, and influences in conjugation and word order. Recent computational studies suggest that early gender shifts may have been motivated by the gender of the corresponding word in Gaulish.");
    mk.train_paragraph("The estimated number of French words that can be attributed to Gaulish is placed at 154 by the Petit Robert, which is often viewed as representing standardized French, while if non-standard dialects are included, the number increases to 240. Known Gaulish loans are skewed toward certain semantic fields, such as plant life (chêne, bille, etc.), animals (mouton, cheval, etc.), nature (boue, etc.), domestic activities (ex. berceau), farming and rural units of measure (arpent, lieue, borne, boisseau), weapons, and products traded regionally rather than further afield. This semantic distribution has been attributed to peasants being the last to hold onto Gaulish.");

    for _ in 0..10 {
        println!("{}", mk.generate_paragraph());
    }
}