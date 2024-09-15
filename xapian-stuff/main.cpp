#include <cstdlib>
#include <fstream>
#include <vector>
#include <xapian.h>

#include <iostream>

using namespace std;

void index(const string & dbpath)
{
  const string title = "The Witch Boy";
  const string isbn = "19573164";
  const string author = "Molly Knox Ostertag";
  const string summary = "In thirteen-year-old Aster's family, all the girls are raised to be witches, while boys grow up to be shapeshifters. Anyone who dares cross those lines is exiled. Unfortunately for Aster, he still hasn't shifted... and he's still fascinated by witchery, no matter how forbidden it might be. When a mysterious danger threatens the other boys, Aster knows he can help -- as a witch. It will take the encouragement of a new friend, the non-magical and non-conforming Charlie, to convince Aster to try practicing his skills. And it will require even more courage to save his family... and be truly himself.";

  Xapian::WritableDatabase db(dbpath, Xapian::DB_CREATE_OR_OPEN);

  Xapian::TermGenerator termgenerator;
  termgenerator.set_stemmer(Xapian::Stem("en"));

  Xapian::Document doc;
  termgenerator.set_document(doc);  

  termgenerator.index_text(title, 1, "S");
  termgenerator.index_text(summary, 1, "B");
  termgenerator.index_text(author, 1, "A");

  termgenerator.index_text(title);
  termgenerator.increase_termpos();
  termgenerator.index_text(summary);

  doc.set_data(isbn + "\n" + title + "by " + author + "\n" + summary);

  string idterm = "Q" + isbn;
  doc.add_boolean_term(idterm);
  db.replace_document(idterm, doc);
}

int main()
{
  index("db");
}
