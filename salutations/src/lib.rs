use std::sync::{mpsc, Arc, Mutex};
use std::thread;

enum Message {
    NouvelleMission(Mission),
    Extinction,
}
/// Structure de gestion de multiples taches
///
/// Chaque Operateur est un receveur du canal envoi
///
pub struct GroupeTaches {
    operateurs: Vec<Operateur>,
    envoi: mpsc::Sender<Message>,
}

type Mission = Box<dyn FnOnce() + Send + 'static>;

impl GroupeTaches {
    /// Crée un nouveau GroupeTaches.
    ///
    /// La taille est le nombre de tâches présentes dans le groupe
    ///
    /// # Panics
    ///
    /// La fonction `new` devrait paniquer si la taille vaut zéro
    pub fn new(taille: usize) -> GroupeTaches {
        assert!(taille > 0);

        let (envoi, reception) = mpsc::channel();

        let reception = Arc::new(Mutex::new(reception));

        let mut operateurs = Vec::with_capacity(taille);

        for id in 0..taille {
            operateurs.push(Operateur::new(id, Arc::clone(&reception)));
        }

        GroupeTaches { operateurs, envoi }
    }

    /// Lance une tache
    ///
    pub fn executer<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let mission = Box::new(f);
        self.envoi.send(Message::NouvelleMission(mission)).unwrap();
    }
}

impl Drop for GroupeTaches {
    fn drop(&mut self) {
        println!("Envoi du message d'extinction à tous les opérateurs.");

        for _ in &self.operateurs {
            self.envoi.send(Message::Extinction).unwrap();
        }

        println!("Arret de tous les opérateurs.");

        for operateur in &mut self.operateurs {
            println!("Arrêt de l'opérateur {}", operateur.id);

            if let Some(tache) = operateur.tache.take() {
                tache.join().unwrap();
            }
        }
    }
}

struct Operateur {
    id: usize,
    tache: Option<thread::JoinHandle<()>>,
}

impl Operateur {
    fn new(id: usize, reception: Arc<Mutex<mpsc::Receiver<Message>>>) -> Operateur {
        let tache = thread::spawn(move || loop {
            let message = reception.lock().unwrap().recv().unwrap();

            match message {
                Message::NouvelleMission(mission) => {
                    println!("L'opérateur {} a reçu une mission, il l'exécute.", id);

                    mission();
                }
                Message::Extinction => {
                    println!("L'opérateur {} a bien reçu ll'instruction d'arrêt", id);

                    break;
                }
            }
        });

        Operateur {
            id,
            tache: Some(tache),
        }
    }
}
